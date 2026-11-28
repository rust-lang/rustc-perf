interface PageData {
  repository: string;
}

// Static data passed by the backend to every page through the #page-data
// element.
export const PAGE_DATA: PageData = JSON.parse(
  document.getElementById("page-data")!.textContent
);
