// Sliding gallery for the Refine screenshots (and any [data-gallery]).
(function () {
  function initGallery(gallery) {
    var track = gallery.querySelector(".gallery-track");
    if (!track) return;
    var slides = Array.prototype.slice.call(track.children);
    if (slides.length === 0) return;

    var prev = gallery.querySelector(".gallery-nav.prev");
    var next = gallery.querySelector(".gallery-nav.next");
    var dotsWrap = gallery.querySelector(".gallery-dots");
    var dots = [];

    function index() {
      return Math.round(track.scrollLeft / track.clientWidth);
    }
    function go(i) {
      var clamped = Math.max(0, Math.min(slides.length - 1, i));
      track.scrollTo({ left: clamped * track.clientWidth, behavior: "smooth" });
    }
    function update() {
      var c = index();
      dots.forEach(function (d, i) {
        d.setAttribute("aria-current", i === c ? "true" : "false");
      });
      if (prev) prev.disabled = c <= 0;
      if (next) next.disabled = c >= slides.length - 1;
    }

    if (dotsWrap && slides.length > 1) {
      slides.forEach(function (_, i) {
        var d = document.createElement("button");
        d.type = "button";
        d.className = "gallery-dot";
        d.setAttribute("aria-label", "Go to screenshot " + (i + 1));
        d.addEventListener("click", function () {
          go(i);
        });
        dotsWrap.appendChild(d);
        dots.push(d);
      });
    }

    if (prev) prev.addEventListener("click", function () { go(index() - 1); });
    if (next) next.addEventListener("click", function () { go(index() + 1); });
    track.addEventListener("scroll", function () {
      window.requestAnimationFrame(update);
    });
    window.addEventListener("resize", update);
    window.addEventListener("stage:change", update);
    update();
  }

  document.querySelectorAll("[data-gallery]").forEach(initGallery);
})();

// Cal.com integration: popups on every booking link + inline embed on /consulting.
(function () {
  var calLink = document.body && document.body.getAttribute("data-cal-slug");
  if (!calLink || !window.Cal) return; // no Cal -> href fallback (opens cal.com)

  // Warm the embed so the overlay opens instantly (and is ready on first click).
  window.Cal("preload", { calLink: calLink });

  // Turn every link to cal.com into a popup overlay instead of a new tab.
  document.querySelectorAll('a[href*="cal.com/"]').forEach(function (a) {
    a.setAttribute("data-cal-link", calLink);
    a.setAttribute("data-cal-config", '{"layout":"month_view"}');
    a.removeAttribute("target");
    // Stop the href from ever navigating — Cal's handler opens the popup.
    // Capture phase guarantees this runs before the default navigation.
    a.addEventListener(
      "click",
      function (e) {
        e.preventDefault();
      },
      true
    );
  });

  // Inline calendar where a #cal-inline container exists.
  if (document.getElementById("cal-inline")) {
    window.Cal("inline", {
      elementOrSelector: "#cal-inline",
      calLink: calLink,
      config: { layout: "month_view" },
    });
  }
})();

// Home stage: one hero with Intro · Work · Writing.
// Steps walk Intro → Work (Growth Lab, Moving Line, Refine) → Writing.
// Roomy screens: the frame pins and ordinary scrolling advances the steps
// (native scroll, never hijacked). Otherwise: plain tabs + arrows + swipe.
// No JS: everything stacks.
(function () {
  var stage = document.querySelector("[data-stage]");
  if (!stage) return;

  var sticky = stage.querySelector(".stage-sticky");
  var tabs = Array.prototype.slice.call(stage.querySelectorAll(".stage-tab"));
  var glow = stage.querySelector(".stage-tab-glow");
  var bar = stage.querySelector(".stage-progress span");
  var panels = Array.prototype.slice.call(stage.querySelectorAll(".stage-panel"));
  var slides = Array.prototype.slice.call(stage.querySelectorAll(".work-slide"));
  var dots = Array.prototype.slice.call(stage.querySelectorAll(".work-dot"));
  var workPrev = stage.querySelector(".work-nav.prev");
  var workNext = stage.querySelector(".work-nav.next");
  var header = document.querySelector(".site-header");
  var reduce = window.matchMedia("(prefers-reduced-motion: reduce)");

  var steps = [{ p: 0 }];
  slides.forEach(function (_, i) { steps.push({ p: 1, s: i }); });
  steps.push({ p: 2 });

  var current = 0;
  var slide = 0; // last Work slide shown, kept when leaving the tab
  var mode = "tabs";
  var stepPx = 0;
  var start = 0;

  function clamp(v, lo, hi) { return Math.max(lo, Math.min(hi, v)); }
  function headerH() { return header ? header.offsetHeight : 0; }

  function setClasses(list, active) {
    list.forEach(function (el, i) {
      el.classList.toggle("is-active", i === active);
      el.classList.toggle("is-before", i < active);
    });
  }

  function moveGlow(i) {
    var t = tabs[i];
    if (!glow || !t) return;
    glow.style.width = t.offsetWidth + "px";
    glow.style.transform = "translateX(" + t.offsetLeft + "px)";
  }

  function apply(i) {
    current = clamp(i, 0, steps.length - 1);
    var st = steps[current];
    if (st.s !== undefined) slide = st.s;
    setClasses(panels, st.p);
    setClasses(slides, slide);
    tabs.forEach(function (t, k) {
      var on = k === st.p;
      t.setAttribute("aria-selected", on ? "true" : "false");
      t.tabIndex = on ? 0 : -1;
    });
    dots.forEach(function (d, k) { d.setAttribute("aria-current", k === slide ? "true" : "false"); });
    if (workPrev) workPrev.disabled = slide <= 0;
    if (workNext) workNext.disabled = slide >= slides.length - 1;
    moveGlow(st.p);
    // the Refine gallery measures its width; nudge it once visible
    window.dispatchEvent(new Event("stage:change"));
  }

  function stepForTab(p) {
    if (p === 1) return 1 + slide;
    for (var i = 0; i < steps.length; i++) if (steps[i].p === p) return i;
    return 0;
  }

  function go(i) {
    i = clamp(i, 0, steps.length - 1);
    if (mode === "pinned") {
      window.scrollTo({ top: start + i * stepPx + 2, behavior: reduce.matches ? "auto" : "smooth" });
    } else {
      apply(i);
    }
  }

  function onScroll() {
    if (mode !== "pinned") return;
    var range = stepPx * (steps.length - 1);
    var progress = clamp((window.scrollY - start) / range, 0, 1);
    if (bar) bar.style.transform = "scaleX(" + progress + ")";
    var i = Math.round(progress * (steps.length - 1));
    if (i !== current) apply(i);
  }

  function layout() {
    var h = headerH();
    stage.style.setProperty("--header-h", h + "px");
    stage.style.height = "";
    stage.classList.remove("is-pinned", "is-tabs");
    mode = "tabs";

    var avail = window.innerHeight - h;
    var roomy = window.innerWidth > 940 && avail >= 600 && !reduce.matches;
    var fits = false;
    if (roomy) {
      // stack every panel in one cell and check the tallest fits one viewport
      stage.classList.add("is-measuring");
      fits = sticky.scrollHeight <= avail;
      stage.classList.remove("is-measuring");
    }
    if (!(roomy && fits)) {
      stage.classList.add("is-tabs");
    } else {
      mode = "pinned";
      stage.classList.add("is-pinned");
      stepPx = Math.round(window.innerHeight * 0.6);
      stage.style.height = avail + stepPx * (steps.length - 1) + "px";
      start = stage.getBoundingClientRect().top + window.scrollY - h;
      onScroll();
    }
    apply(current);
  }

  // tabs
  tabs.forEach(function (t, k) {
    t.addEventListener("click", function () { go(stepForTab(k)); });
    t.addEventListener("keydown", function (e) {
      var d = e.key === "ArrowRight" ? 1 : e.key === "ArrowLeft" ? -1 : 0;
      if (!d) return;
      e.preventDefault();
      var n = clamp(steps[current].p + d, 0, tabs.length - 1);
      go(stepForTab(n));
      tabs[n].focus();
    });
  });

  // work carousel controls
  dots.forEach(function (d, k) { d.addEventListener("click", function () { go(1 + k); }); });
  if (workPrev) workPrev.addEventListener("click", function () { go(1 + slide - 1); });
  if (workNext) workNext.addEventListener("click", function () { go(1 + slide + 1); });

  // swipe (tabs mode): left/right steps through, ignoring the Refine gallery
  var sx = null, sy = null;
  stage.addEventListener("touchstart", function (e) {
    if (e.target.closest(".gallery-track")) { sx = null; return; }
    sx = e.touches[0].clientX; sy = e.touches[0].clientY;
  }, { passive: true });
  stage.addEventListener("touchend", function (e) {
    if (sx === null || mode !== "tabs") return;
    var dx = e.changedTouches[0].clientX - sx, dy = e.changedTouches[0].clientY - sy;
    if (Math.abs(dx) > 50 && Math.abs(dx) > Math.abs(dy) * 1.5) go(current + (dx < 0 ? 1 : -1));
    sx = null;
  }, { passive: true });

  window.addEventListener("scroll", function () { window.requestAnimationFrame(onScroll); }, { passive: true });
  var rt;
  window.addEventListener("resize", function () { clearTimeout(rt); rt = setTimeout(layout, 120); });
  if (reduce.addEventListener) reduce.addEventListener("change", layout);
  window.addEventListener("load", layout);
  layout();
})();
