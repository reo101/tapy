/**
 * @name Tapy
 * @author reo101
 * @description A tool for tagging and quickly accessing all kinds of media
 * @version 0.0.1
 * @authorId 237187608826871808
 * @authorLink https://www.github.com/reo101
 * @website https://www.github.com/reo101/tapy/tree/master/betterdiscord_plugin
 * @source https://raw.githubusercontent.com/reo101/tapy/master/betterdiscord_plugin/dist/tapy.plugin.js
 */

import { renderToStaticMarkup } from "react-dom/server";

const config = {
  info: {
    name: "Tapy (Tag and Copy)",
    authors: [
      {
        name: "reo101",
        discord_id: "237187608826871808",
        github_username: "reo101",
      },
    ],
    version: "0.0.1",
    description: "A tool for tagging and quickly accessing all kinds of media",
    github:
      "https://www.github.com/reo101/tapy/tree/master/betterdiscord_plugin",
    github_raw:
      "https://raw.githubusercontent.com/reo101/tapy/master/betterdiscord_plugin/dist/tapy.plugin.js",
  },
  changelog: [],
};

//META{"name":"Tapy"}*//
module.exports = class Tapy {
  /**
   * @name getName
   * The name for the plugin to be displayed to the user in
   * the plugins page and for internal settings to use.
   *
   * @returns {string} - the name for the plugin.
   */
  getName(): string {
    return config.info.name;
  }

  /**
   * @name getDescription
   * The description of the plugin shown in the plugins page.
   *
   * @returns {string} - the description of the plugin.
   */
  getDescription(): string {
    return config.info.description;
  }

  /**
   * @name getVersion
   * The version of the plugin displayed in the plugins page.
   *
   * @returns {string} - the version of the plugin.
   */
  getVersion(): string {
    return config.info.version;
  }

  /**
   * @name getAuthor
   * The author string for the plugin displayed in the plugins page.
   *
   * @returns {string} - the author of the plugin.
   */
  getAuthor(): string {
    return config.info.authors[0].name;
  }

  /**
   * @name getGithub
   * The Github repository for the plugin
   *
   * @returns {string} - the repository url
   */
  getGithub(): string {
    return config.info.github;
  }

  /**
   * @name getGithubRaw
   * The raw plugin file location
   *
   * @returns {string} - the plugin file url
   */
  getGithubRaw(): string {
    return config.info.github_raw;
  }

  /**
   * @name load
   * Called when the plugin is loaded regardless of if it is enabled or disabled.
   */
  load() {}

  /**
   * @name start
   * Called when the plugin is enabled or when it is loaded and was previously
   * reloaded (such as discord start or reload).
   */
  start() {}

  /**
   * @name stop
   * Called when the plugin is disabled.
   */
  stop() {}

  /**
   * @name observer
   * Called on every mutation that occurs on the document.
   * For more information on observers and mutations take a look at MDN's documentation.
   * https://developer.mozilla.org/en-US/docs/Web/API/MutationObserver
   *
   * @param {MutationObserver} changes - The mutation that occurred.
   */
  observer(changes: MutationObserver) {}

  /**
   * @name onSwitch
   * Called every time the user navigates such as changing channel,
   * changing servers, changing to friends list, etc.
   */
  onSwitch() {}

  /**
   * @name getSettingsPanel
   * Called when the user clicks on the settings button for the plugin.
   * If this function is not implemented the button is not shown.
   *
   * Note: The button will be disabled if the plugin is disabled to avoid errors with not-started plugins.
   *
   * @returns {string|HTMLElement} - either a valid string containing the html for the panel or an actual
   * element to be injected into the settings panel.
   */
  getSettingsPanel(): string | HTMLElement {
    return renderToStaticMarkup(
      <>
        <h3>Settings Panel</h3>
      </>
    );
  }
};
