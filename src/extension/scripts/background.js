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

const _activateIcon = () => chrome.action.setIcon({
  path: {
    "16": "../images/icon16.png",
    "32": "../images/icon32.png"
  }
});
const _deactivateIcon = () => chrome.action.setIcon({
  path: {
    "16": "../images/icon16_disabled.png",
    "32": "../images/icon32_disabled.png"
  }
});

const updateIcon = (url) =>  (typeof (url) == "string" && url.match(/https:\/\/musescore.com\/.*\/scores\/.*/g)) ? _activateIcon() : _deactivateIcon();
  
chrome.tabs.onActivated.addListener(async (activeInfo) => {
  let [tab] = await chrome.tabs.query({ active: true, lastFocusedWindow: true });
  if (!tab) return;
  let url = tab.url ?? tab.pending;
  updateIcon(url);
});
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.url) updateIcon(changeInfo['url']);
});


chrome.downloads.onChanged.addListener((downloadDelta) => {
  console.log(downloadDelta);
  if (downloadDelta.filename) {
    let match = downloadDelta.filename.current.match(/.*score_([0-9]*).*.(png|svg)/);
    console.log(match);
    if (match) { console.log(`Downloading file #${match[1]}`); }
  }
});

chrome.contentSettings.automaticDownloads.set({
  primaryPattern: "https://musescore.com/*",
  setting: "allow"
});

chrome.action.onClicked.addListener((tab) => {
  chrome.scripting.executeScript({
    target: { tabId: tab.id },
    files: ['scripts/content.js']
  });
});
