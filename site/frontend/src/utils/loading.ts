import {Ref} from "vue";

/**
 * Manages a loading variable for an asynchronous function.
 */
export async function withLoading<T>(
  loading: Ref<boolean>,
  callback: () => Promise<T>
): Promise<T> {
  loading.value = true;
  try {
    return await callback();
  } finally {
    loading.value = false;
  }
}
