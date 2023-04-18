// Parts of this file taken from https://github.com/rust-lang/cargo,
// specifically src/cargo/core/compiler/timings.js. That content is licensed as
// dual MIT/Apache 2.0 as well.

let thread_height = 18;
let margin = { x: 20, y: 6 };
let event_paths = [];
let px_per_micros = 0;

function roundedRect(x, y, width, height, r) {
    let path = new Path2D();
    r = Math.min(r, width, height);
    width -= r*2;
    height -= r*2;
    path.moveTo(x+r, y);
    path.lineTo(x+r+width, y);
    path.arc(x+r+width, y+r, r, -Math.PI/2, 0);
    path.lineTo(x+r+width+r, y+r);
    path.lineTo(x+r+width+r, y+r+height);
    path.arc(x+r+width, y+height+r, r, 0, Math.PI/2);
    path.lineTo(x+r, y+r+height+r);
    path.arc(x+r, y+r+height, r, Math.PI/2, Math.PI);
    path.lineTo(x, y+r);
    path.arc(x+r, y+r, r, Math.PI, 3*Math.PI/2);
    path.closePath();
    return path;
}

function setup(canvas_display_id, by_thread, tagline) {
    let primary = document.getElementById(canvas_display_id);
    let ctx = primary.getContext('2d');

    const threads = Object.keys(by_thread).map(k => parseInt(k, 10));
    threads.sort((a, b) => a - b);
    const canvas_height = Math.ceil(thread_height*(threads.length+1))+margin.y*2;

    let scale = (document.body.getBoundingClientRect().width - margin.x*2.3) / TOTAL_DURATION_BOTH * 50;
    let env = { tagline, event_paths: [] };

    render(env, threads, by_thread, primary, canvas_height, ctx, scale);

    let isShown = null;
    primary.addEventListener("mousemove", e => {
        let dpr = window.devicePixelRatio || 1;
        for (let {event, path, thread} of env.event_paths) {
            if (ctx.isPointInPath(path, e.offsetX * dpr, e.offsetY * dpr)) {
                if (isShown != event) {
                    env.suffix = ` ${event.name} took ${event.end-event.start}ms`;
                    render(env, threads, by_thread, primary, canvas_height, ctx, scale);
                    ctx.strokeStyle = 'red';
                    ctx.stroke(path);

                    ctx.beginPath();
                    ctx.setLineDash([3, 10]);
                    ctx.moveTo(margin.x + px_per_micros*by_thread[thread][0].start, margin.y);
                    ctx.lineTo(margin.x + px_per_micros*by_thread[thread][0].start, canvas_height-margin.y);

                    if (event != by_thread[thread][0]) {
                        ctx.moveTo(margin.x + px_per_micros*event.start, margin.y);
                        ctx.lineTo(margin.x + px_per_micros*event.start, canvas_height-margin.y);
                    }

                    ctx.stroke();
                    ctx.setLineDash([]);

                    isShown = event;
                }
                return;
            }
        }
    });

    primary.style.display = "block";
}

function render(env, threads, by_thread, primary, canvas_height, ctx, scale) {
    const canvas_width = Math.min(4096, margin.x*2 + (scale / 50) * TOTAL_DURATION_BOTH);
    let dpr = window.devicePixelRatio || 1;
    primary.width = canvas_width * dpr;
    primary.height = canvas_height * dpr;
    primary.style.width = canvas_width;
    primary.style.height = canvas_height;
    ctx.clearRect(0, 0, canvas_width, canvas_height);
    ctx.scale(dpr, dpr);

    px_per_micros = (canvas_width - margin.x*2) / TOTAL_DURATION_BOTH;

    env.event_paths = [];

    ctx.textBaseline="top";

    ctx.fillStyle = 'black';
    ctx.fillText(env.tagline + (env.suffix || ""), 0, 0 + 2);

    for (let i = 0; i < threads.length; i++) {
        let thread = threads[i];
        let events = by_thread[thread];
        ctx.fillStyle = 'black';
        ctx.fillText(thread, 0, topleft_thread_box(margin, thread_height, i) + 2);

        ctx.beginPath();
        ctx.strokeStyle = '#ededed';
        ctx.moveTo(margin.x, topleft_thread_box(margin, thread_height, i));
        ctx.lineTo(margin.x + px_per_micros*TOTAL_DURATION_BOTH, topleft_thread_box(margin, thread_height, i));
        ctx.stroke();

        for (let event of events) {
            let eventPath = roundedRect(
                margin.x+px_per_micros*event.start,
                topleft_thread_box(margin, thread_height, i) + 2,
                px_per_micros*(event.end-event.start),
                thread_height-4,
                8,
            );
            env.event_paths.push({ event, path: eventPath, thread });

            ctx.fillStyle = color(event.name);
            ctx.fill(eventPath);
        }
    }

    ctx.strokeStyle = 'lightgrey';
    ctx.stroke(roundedRect(margin, topleft_thread_box(margin, thread_height, 0), px_per_micros*TOTAL_DURATION_BOTH, threads.length * thread_height, 0));
}

function color(name) {
    switch (name) {
        case "codegen_module_optimize":
            return '#aa95e8';
        case "codegen_module_perform_lto":
            return '#428eff';
        case "codegen_module":
            return '#6adb00';
    }
}

function topleft_thread_box(margin, thread_height, i) {
    return margin.y + (i+1)*thread_height;
}

window.addEventListener("DOMContentLoaded", () => {
    setup("current", BY_THREAD, "Current:");
    if (BY_THREAD_BASE) {
        setup("prev", BY_THREAD_BASE, "Baseline:");
    }
});
