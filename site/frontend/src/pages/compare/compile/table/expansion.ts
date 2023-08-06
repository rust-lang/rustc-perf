import {ref} from "vue";
import {CompileTestCase} from "../common";

export function useExpandedStore() {
  const expanded = ref(new Set());

  function isExpanded(testCase: CompileTestCase) {
    return expanded.value.has(testCaseKey(testCase));
  }

  function toggleExpanded(testCase: CompileTestCase) {
    const key = testCaseKey(testCase);
    if (isExpanded(testCase)) {
      expanded.value.delete(key);
    } else {
      expanded.value.add(key);
    }
  }

  return {toggleExpanded, isExpanded};
}

function testCaseKey(testCase: CompileTestCase): string {
  return `${testCase.benchmark};${testCase.profile};${testCase.scenario};${testCase.category}`;
}
