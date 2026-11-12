import importlib.util
import threading
import time
import unittest
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path
from unittest.mock import patch

import requests


spec = importlib.util.spec_from_file_location(
    "check_site", Path(__file__).with_name("check-site.py")
)
check_site = importlib.util.module_from_spec(spec)
spec.loader.exec_module(check_site)


class SiteReadinessTests(unittest.TestCase):
    def serve(self, replies):
        class Handler(BaseHTTPRequestHandler):
            def do_POST(self):
                self.rfile.read(int(self.headers["Content-Length"]))
                status, body = replies.pop(0) if len(replies) > 1 else replies[0]
                self.send_response(status)
                self.end_headers()
                try:
                    self.wfile.write(body)
                except (BrokenPipeError, ConnectionResetError):
                    # Deadline tests can close the connection before the server writes.
                    pass

            def log_message(self, *args):
                pass

        server = ThreadingHTTPServer(("127.0.0.1", 0), Handler)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        self.addCleanup(server.server_close)
        self.addCleanup(server.shutdown)
        return f"http://127.0.0.1:{server.server_port}/perf/get"

    def wait(self, url, timeout=1):
        return check_site.wait_for_site(
            "version1", "version2", url=url, timeout=timeout, poll_interval=0.01
        )

    def test_ready_site_returns(self):
        self.wait(self.serve([(200, b"ready")]))

    def test_initializing_site_is_retried(self):
        replies = [
            (200, b"no data yet, please wait"),
            (200, b"no data yet, please wait"),
            (200, b"ready"),
        ]
        self.wait(self.serve(replies))
        self.assertEqual(len(replies), 1)

    def test_http_error_is_not_treated_as_readiness(self):
        replies = [(503, b"unavailable"), (503, b"unavailable"), (200, b"ready")]
        self.wait(self.serve(replies))
        self.assertEqual(len(replies), 1)

    def test_initializing_site_has_a_deadline(self):
        url = self.serve([(200, b"no data yet, please wait")])
        start = time.monotonic()
        with self.assertRaises(TimeoutError):
            self.wait(url, timeout=0.05)
        self.assertLess(time.monotonic() - start, 1)

    def test_connection_failures_have_a_deadline(self):
        # The failure is at the external HTTP boundary; the retry loop is real.
        with patch.object(
            check_site.requests, "post", side_effect=requests.ConnectionError("offline")
        ):
            with self.assertRaises(TimeoutError):
                self.wait("http://localhost:2346/perf/get", timeout=0.03)

    def test_request_read_timeout_is_bounded_by_remaining_time(self):
        observed_timeouts = []

        def post(*args, **kwargs):
            observed_timeouts.append(kwargs.get("timeout"))
            response = requests.Response()
            response.status_code = 200
            response._content = b"ready"
            return response

        with patch.object(check_site.requests, "post", side_effect=post), patch.object(
            check_site.time, "monotonic", side_effect=[0, 0.05, 0.06]
        ):
            self.wait("http://localhost:2346/perf/get", timeout=0.2)
        self.assertAlmostEqual(observed_timeouts[0], 0.15)

    def test_ready_response_after_retry_deadline_is_rejected(self):
        response = requests.Response()
        response.status_code = 200
        response._content = b"ready"
        with patch.object(check_site.requests, "post", return_value=response), patch.object(
            check_site.time, "monotonic", side_effect=[0, 0.01, 0.11]
        ):
            with self.assertRaises(TimeoutError):
                self.wait("http://localhost:2346/perf/get", timeout=0.1)

    def test_keyboard_interrupt_is_not_swallowed(self):
        with patch.object(check_site.requests, "post", side_effect=KeyboardInterrupt):
            with self.assertRaises(KeyboardInterrupt):
                self.wait("http://localhost:2346/perf/get")

    def test_unexpected_programming_error_is_not_retried(self):
        with patch.object(check_site.requests, "post", side_effect=ValueError("bug")):
            with self.assertRaisesRegex(ValueError, "bug"):
                self.wait("http://localhost:2346/perf/get")


if __name__ == "__main__":
    unittest.main()
