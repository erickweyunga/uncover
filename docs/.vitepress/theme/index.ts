import DefaultTheme from "vitepress/theme";
import "./custom.css";
import type { Theme } from "vitepress";
import CopyOrDownloadAsMarkdownButtons from "vitepress-plugin-llms";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    app.component(
      "CopyOrDownloadAsMarkdownButtons",
      CopyOrDownloadAsMarkdownButtons,
    );
  },
} satisfies Theme;
