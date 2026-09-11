import { actions } from "./datastar.js";

export function normalizeRoute(path) {
  var route = (path || "/").split(/[?#]/)[0];
  if (!route || route === "/") {
    return "/";
  }
  return "/" + route.replace(/^\/+|\/+$/g, "") + "/";
}

export function isInAppDocumentHref(href, originHref) {
  if (!href || href.indexOf("/__okmate") === 0) {
    return false;
  }
  var dest;
  try {
    dest = new URL(href, originHref || (window.location && window.location.href) || "http://okmate.local/");
  } catch (err) {
    return false;
  }
  if (dest.protocol === "mailto:" || dest.protocol === "javascript:") {
    return false;
  }
  if (dest.origin && window.location && dest.origin !== window.location.origin) {
    return false;
  }
  var path = dest.pathname || "";
  if (path.indexOf("/assets/") === 0 || /\.(png|jpe?g|gif|svg|webp|pdf)$/i.test(path)) {
    return false;
  }
  return true;
}

export function requestDocument(href) {
  if (!href) {
    return;
  }
  var el = document.getElementById("okmate-ds");
  if (!el) {
    return;
  }
  actions.get({ el: el }, href);
}
