# Okmate

Application, desktop preview, agent setup, and product site.

* [Support heterogeneous OKF bundle authoring](heterogeneous-bundle-authoring.md) - Minimal and selectable init, correct paths, separate evidence/style checks, local conventions, discovery, and measured curation. Research: [bundle structure](/research/okmate/heterogeneous-bundle-authoring.md). Exploratory; Phases 1–5 in this revision; discovery decision approved.
* [Initialize a new OKF bundle](init-bundle.md) - Original design for the now-implemented `init` scaffold, registration, and agent extras; phase completion is not certified here. Follow-up: [heterogeneous authoring](heterogeneous-bundle-authoring.md).
* [Implementation structure](implementation-structure.md) - Split oversized modules, shrink the okf public surface, type page kinds, take session I/O off the GET path. No new features. Audit: [implementation-structure](/audits/okmate/implementation-structure.md). Exploratory; no phase started.
* [Verify and promote from the review UI](verify-promote.md) - Loopback Verify/Promote, git working tree; bundle inference must follow the approved discovery decision (no `knowledge/` privilege). Research: [review-queue authoring](/research/okmate/review-queue-authoring.md). Exploratory; no phase started.
* [OKMate product website](website.md) - Static Rocdown site: OSS/OKF/git/multi-bundle story, human docs, `/agents/` plus `/llms.txt`. Research: [website](/research/okmate/website.md). Exploratory; no phase started.
* [Agent and knowledge bootstrap](agent-knowledge.md) - Skills, Cursor rules, local bundle, and discussion migration from Rocci.
* [Standalone app self-update](standalone-self-update.md) - Sparkle 2 in OKMate.app; GitHub Releases feed.
* [Dashboard parity with rocci-okf](dashboard-parity.md) - Home governance, review queue, concept meta, and preview ports.
* [Viewer shell parity with last rocci-okf](viewer-shell-parity.md) - Three panes, resize, outline spy, and keep-nav sidebar.
* [Extended multi-bundle viewer](extended-multi-bundle.md) - Workspace nav modes, merged dashboard recents and log, collection hover.
* [Viewer responsiveness](viewer-responsiveness.md) - `okmate timings` pipeline, in-memory clicks, preview load policy, windowed review and log.
* [Peek previews and document tabs](peek-and-tabs.md) - Heading and keyed-footnote hover peeks, then a live-preview tab strip. Research: [reference authoring](/research/okf/reference-authoring-style.md). Exploratory; no phase started.
* [Idiomatic document tab gestures](tab-gestures.md) - Current-tab navigation, Cmd+T home, desktop Cmd+W closes a tab; document titles and type-color dots. Extends [peek-and-tabs](peek-and-tabs.md). Exploratory; Phases 1–4 implemented.
* [Datastar-aligned client JavaScript](datastar-client-js.md) - ES modules, public `@get`, `datastar-fetch`, enhance Askama tabs. Research: [datastar-client-js](/research/okmate/datastar-client-js.md). Exploratory; no phase started.
