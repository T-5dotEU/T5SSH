let nextId = 1;

function createTabState() {
  let tabs = $state([]);
  let activeTabId = $state(null);

  return {
    get tabs() { return tabs; },
    get activeTabId() { return activeTabId; },
    set activeTabId(id) { activeTabId = id; },

    addTab(profile) {
      const id = nextId++;
      const label = profile.user
        ? `${profile.user}@${profile.host}`
        : profile.host;

      const tab = {
        id,
        label,
        profile,
        sessionId: null,
        disconnected: false,
      };

      tabs.push(tab);
      activeTabId = id;
      return id;
    },

    removeTab(id) {
      const idx = tabs.findIndex((t) => t.id === id);
      if (idx === -1) return;

      tabs.splice(idx, 1);

      if (activeTabId === id) {
        if (tabs.length > 0) {
          const newIdx = Math.min(idx, tabs.length - 1);
          activeTabId = tabs[newIdx].id;
        } else {
          activeTabId = null;
        }
      }
    },

    setActiveTab(id) {
      activeTabId = id;
    },

    getTab(id) {
      return tabs.find((t) => t.id === id);
    },

    setSessionId(tabId, sessionId) {
      const tab = tabs.find((t) => t.id === tabId);
      if (tab) tab.sessionId = sessionId;
    },

    markDisconnected(sessionId) {
      const tab = tabs.find((t) => t.sessionId === sessionId);
      if (tab) tab.disconnected = true;
    },
  };
}

export const tabStore = createTabState();
