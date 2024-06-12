export const saveData = (key, data) => {
  if (isChromeExtension()) {
    try {
      DiChrome.storage.local.set({ [key]: data });
    } catch (err) {
      console.error("Error saving to local state");
      console.error(err);
    }
  } else {
    return Promise.resolve(localStorage.setItem(key, JSON.stringify(data)));
  }
};

export const loadData = (key) => {
  if (isChromeExtension()) {
    try {
      return chrome.storage.local.get(key).then((data) => data[key]);
    } catch (err) {
      console.error("Error while loading the data from the local state");
      console.error(err);
    }
  } else {
    return Promise.resolve(JSON.parse(localStorage.getItem(key)));
  }
};

const isChromeExtension = () => {
  return !!chrome?.storage;
};
