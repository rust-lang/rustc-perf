export interface MetricDescription {
  label: string;
  metric: string;
  description: string;
}

const sharedMetrics: MetricDescription[] = [
  {
    label: "Instructions",
    metric: "instructions:u",
    description: "Number of executed instructions",
  },
  {
    label: "Cycles",
    metric: "cycles:u",
    description: "Number of executed cycles",
  },
  {
    label: "Wall time",
    metric: "wall-time",
    description: "Wall time",
  },
];

export const importantCompileMetrics: MetricDescription[] = [
  ...sharedMetrics,
  {
    label: "Max RSS",
    metric: "max-rss",
    description: "Peak memory usage (resident set size)",
  },
  {
    label: "Binary size",
    metric: "size:linked_artifact",
    description: "Size of the generated binary artifact",
  },
];

export const importantRuntimeMetrics: MetricDescription[] = [...sharedMetrics];
