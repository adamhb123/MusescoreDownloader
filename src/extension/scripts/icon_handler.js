const icon_handler = () => {
    const url = window.location.href;
    console.log("CUR_URL: " + url);
    if (url.startsWith("https://musescore.com") && url.includes("/scores/")) {
        console.log("YUP");
        chrome.runtime.sendMessage({
            type: "musescore-downloader", options: {
                icon_update: "enable"
            }
        });

    } else {
        console.log("NOPE");
        chrome.runtime.sendMessage({
            type: "musescore-downloader", options: {
                icon_update: "disable"
            }
        });
    }
};