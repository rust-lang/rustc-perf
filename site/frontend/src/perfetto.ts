// Code for displaying post-processed self-profile traces in perfetto.dev.
// The process of linking to perfetto is explained at
// https://perfetto.dev/docs/visualization/deep-linking-to-perfetto-ui.
// This code is adapted from https://llvm-compile-time-tracker.com/compare.php
const ORIGIN = "https://ui.perfetto.dev";

export async function openTraceInPerfetto(traceUrl: string, title: string) {
  const resp = await fetch(traceUrl);
  const blob = await resp.blob();
  const arrayBuffer = await blob.arrayBuffer();
  openTrace(arrayBuffer, title);
}

function openTrace(arrayBuffer: ArrayBuffer, title: string) {
  const win = window.open(ORIGIN);
  if (win === null) {
    return;
  }

  const timer = setInterval(() => win.postMessage("PING", ORIGIN), 50);

  const onMessageHandler = (evt) => {
    if (evt.data !== "PONG") return;

    // We got a PONG, the UI is ready.
    window.clearInterval(timer);
    window.removeEventListener("message", onMessageHandler);

    win.postMessage(
      {
        perfetto: {
          buffer: arrayBuffer,
          title,
        },
      },
      ORIGIN
    );
  };

  window.addEventListener("message", onMessageHandler);
}
