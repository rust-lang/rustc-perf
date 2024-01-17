"""
Checks that the perf site running locally returns non-empty data for a set of artifacts.
"""
import sys
import time

import msgpack
import requests

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python3 check-site.py <version1> <version2>")
        exit(1)
    version1 = sys.argv[1]
    version2 = sys.argv[2]

    # Wait for the site to start
    while True:
        try:
            response = requests.post("http://localhost:2346/perf/get", json={
                "start": version1,
                "end": version2,
                "stat": "instructions:u"
            })
            if response.content != b"no data yet, please wait":
                break
        except BaseException as e:
            print(e)

        print(f"Site not online yet, waiting")
        time.sleep(1)

    # instructions:u is not available on CI, so check at least wall time and binary size
    stats = ("wall-time", "size:linked_artifact")
    for stat in stats:
        print(f"Checking {stat}")
        response = requests.post("http://localhost:2346/perf/get", json={
            "start": version1,
            "end": version2,
            "stat": stat
        })
        if response.status_code != 200:
            raise Exception(f"Failure {response.status_code}: {response.content}")
        payload = msgpack.unpackb(response.content)
        print(payload)
        for artifact_id in ("a", "b"):
            artifact = payload[artifact_id]
            assert artifact["component_sizes"].get("librustc_driver", 0) > 0
        comparisons = payload["compile_comparisons"]
        assert len(comparisons) > 0
