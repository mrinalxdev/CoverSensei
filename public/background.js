// global chrome

const linkedInListViewURL = "https://www.linkedin.com/jobs/collections";
const linkedinDetailView = "https://www.linkedin.com/jobs/view";

function getJobDecriptionClassName(url) {
  return url.startsWith(linkedInListViewURL)
    ? "jobs-search_job-details--container"
    : "jobs-description-content__text";
}

function grabJobDescription(className) {
  const jobDetailsContainer = document.body.querySelector(`.${className}`);
  const jobDetails = jobDetailsContainer.textContent;
  const cleanedJobDetails = jobDetails.replace(/\s\s+/g, " ");

  return cleanedJobDetails;
}

chrome.tabs.onUpdated.addListener(function (tabId, changeInfo, tab) {
  if (changeInfo.status == "complete" && tab.active) {
    if (
      tab.url?.startsWith(linkedInListViewURL) ||
      tab.url?.startsWith(linkedInListViewURL)
    ) {
      chrome.scripting
        .executeScript({
          target: { tabId: tabId },
          func: grabJobDescription,
          args: [grabJobDescriptionClassName(tab.url)],
        })
        .then((queryResult) => {
          chrome.storage.local.set({ jobDescription: queryResult[0].result });
        });
    }
  }
});
