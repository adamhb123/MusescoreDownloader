// chrome.runtime.onInstalled.addListener(() => {
//   // Page actions are disabled by default and enabled on select tabs
//   chrome.action.disable();
//   chrome.action.setIcon({
//     path: {
//       "16": "../images/icon16_disabled.png",
//       "32": "../images/icon32_disabled.png"
//     }
//   });

//   // Clear all rules to ensure only our expected rules are set
//   chrome.declarativeContent.onPageChanged.removeRules(undefined, () => {
//     // Declare a rule to enable the action on musescore score page
//     let rule = {
//       conditions: [
//         new chrome.declarativeContent.PageStateMatcher({
//           pageUrl: { hostSuffix: 'musescore.com', pathContains: "/scores/" },
//         })
//       ],
//       actions: [new chrome.declarativeContent.ShowAction()]
//     }
//     chrome.declarativeContent.onPageChanged.addRules([rule]);
//   });
// });
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  console.log(changeInfo);
  if ("url" in changeInfo && typeof (changeInfo["url"]) == "string") {
    if (changeInfo['url'].includes("https://musescore")) {
      chrome.action.setIcon({
        path: {
          "16": "../images/icon16.png",
          "32": "../images/icon32.png"
        }
      });
    }
    else {
      chrome.action.setIcon({
        path: {
          "16": "../images/icon16_disabled.png",
          "32": "../images/icon32_disabled.png"
        }
      });
    }
  }
});
chrome.action.onClicked.addListener((tab) => {
  chrome.contentSettings.automaticDownloads.set({
    primaryPattern: "https://musescore.com/*/scores/*",
    setting: "allow"
  })
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    files: ['content.js']
  });
});
