import {ProfileEnumeration} from "../../common";

// Normalize profile from a test case to a CLI value for collector.
export function normalizeProfile(profile: ProfileEnumeration): string {
  if (profile === "opt") {
    return "Opt";
  } else if (profile === "debug") {
    return "Debug";
  } else if (profile === "check") {
    return "Check";
  } else if (profile === "doc") {
    return "Doc";
  }
  return "<invalid profile>";
}
