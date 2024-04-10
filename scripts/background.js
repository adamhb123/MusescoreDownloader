chrome.runtime.onInstalled.addListener(() => {
    // Page actions are disabled by default and enabled on select tabs
    chrome.action.disable();
  
    // Clear all rules to ensure only our expected rules are set
    chrome.declarativeContent.onPageChanged.removeRules(undefined, () => {
      // Declare a rule to enable the action on example.com pages
      let rule = {
        conditions: [
          new chrome.declarativeContent.PageStateMatcher({
            pageUrl: {hostSuffix: 'musescore.com', pathContains: "/scores/"},
          })
        ],
        actions: [new chrome.declarativeContent.ShowAction()],
      };
      chrome.declarativeContent.onPageChanged.addRules([rule]);
    });
  });

chrome.action.onClicked.addListener((tab) => {
    chrome.scripting.executeScript({
        target: { tabId: tab.id },
        files: ['scripts/content.js']
    });
});
