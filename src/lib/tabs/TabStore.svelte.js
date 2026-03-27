/**
 * @typedef {{
 *   id: number,
 *   label: string,
 *   profile: Record<string, any>,
 *   password: string|null,
 *   profileName: string|null,
 *   sessionId: string|null,
 *   disconnected: boolean
 * }} Tab
 */

let nextId = 1;

function createTabState() {
  /** @type {Tab[]} */
  let tabs = $state([]);
  /** @type {number|null} */
  let activeTabId = $state(null);

  return {
    get tabs() { return tabs; },
    get activeTabId() { return activeTabId; },
    set activeTabId(id) { activeTabId = id; },

    /**
     * @param {Record<string, any>} profile
     * @param {string|null} [password]
     * @param {string|null} [profileName]
     */
    addTab(profile, password = null, profileName = null) {
      const id = nextId++;
      const label = profile.user
        ? `${profile.user}@${profile.host}`
        : profile.host;

      const tab = {
        id,
        label,
        profile,
        password,
        profileName,
        sessionId: null,
        disconnected: false,
      };

      tabs.push(tab);
      activeTabId = id;
      return id;
    },

    /** @param {number} id */
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

    /** @param {number} id */
    setActiveTab(id) {
      activeTabId = id;
    },

    /** @param {number} id */
    getTab(id) {
      return tabs.find((t) => t.id === id);
    },

    /**
     * @param {number} tabId
     * @param {string} sessionId
     */
    setSessionId(tabId, sessionId) {
      const tab = tabs.find((t) => t.id === tabId);
      if (tab) tab.sessionId = sessionId;
    },

    /** @param {string} sessionId */
    markDisconnected(sessionId) {
      const tab = tabs.find((t) => t.sessionId === sessionId);
      if (tab) tab.disconnected = true;
    },

    /** @param {number} tabId */
    markReconnecting(tabId) {
      const tab = tabs.find((t) => t.id === tabId);
      if (tab) tab.disconnected = false;
    },
  };
}

export const tabStore = createTabState();
