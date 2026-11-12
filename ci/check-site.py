"""
Checks that the perf site running locally returns non-empty data for a set of artifacts.
"""
import sys
import time

import msgpack
import requests


SITE_URL = "http://localhost:2346/perf/get"
REQUEST_TIMEOUT = 10


def wait_for_site(version1, version2, *, url=SITE_URL, timeout=120, poll_interval=1):
    """Limit startup retries, with a separate inactivity timeout for each request.

    Requests' timeout does not impose a total response-download deadline.
    """
    deadline = time.monotonic() + timeout
    while True:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError(f"Site did not become ready within {timeout} seconds")
        try:
            response = requests.post(url, json={
                "start": version1,
                "end": version2,
                "stat": "instructions:u"
            }, timeout=min(REQUEST_TIMEOUT, remaining))
            if time.monotonic() >= deadline:
                raise TimeoutError(f"Site did not become ready within {timeout} seconds")
            if response.status_code == 200 and response.content != b"no data yet, please wait":
                return
        except requests.RequestException as e:
            print(e)

        print("Site not online yet, waiting")
        time.sleep(min(poll_interval, max(0, deadline - time.monotonic())))


if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 check-site.py <version1> <version2>")
        exit(1)
    version1 = sys.argv[1]
    version2 = sys.argv[2]

    # Wait for the site to start
    wait_for_site(version1, version2)

    # instructions:u is not available on CI, so check at least wall time and binary size
    stats = ("wall-time", "size:linked_artifact")
    for stat in stats:
        print(f"Checking {stat}")
        response = requests.post(SITE_URL, json={
            "start": version1,
            "end": version2,
            "stat": stat
        }, timeout=REQUEST_TIMEOUT)
        if response.status_code != 200:
            raise Exception(f"Failure {response.status_code}: {response.content}")
        payload = msgpack.unpackb(response.content)
        print(payload)
        for artifact_id in ("a", "b"):
            artifact = payload[artifact_id]
            assert artifact["component_sizes"].get("librustc_driver", 0) > 0
        comparisons = payload["compile_comparisons"]
        assert len(comparisons) > 0
