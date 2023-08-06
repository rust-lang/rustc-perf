import {ArtifactDescription} from "../../types";

/**
 * Returns a date in the past for which we want to display a historical chart.
 */
export function getDateInPast(artifact: ArtifactDescription): string {
  const date = new Date(artifact.date);
  // Move to `30 days ago` to display history of the test case
  date.setUTCDate(date.getUTCDate() - 30);
  const year = date.getUTCFullYear();
  const month = (date.getUTCMonth() + 1).toString().padStart(2, "0");
  const day = date.getUTCDate().toString().padStart(2, "0");
  return `${year}-${month}-${day}`;
}
