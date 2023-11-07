import {ref} from "vue";

export function useExpandedStore() {
  const expanded = ref(new Set());

  function isExpanded(sha: string) {
    return expanded.value.has(sha);
  }

  function toggleExpanded(sha: string) {
    if (isExpanded(sha)) {
      expanded.value.delete(sha);
    } else {
      expanded.value.add(sha);
    }
  }

  return {toggleExpanded, isExpanded};
}
