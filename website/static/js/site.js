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
    update();
  }

  document.querySelectorAll("[data-gallery]").forEach(initGallery);
})();

// Cal.com integration: popups on every booking link + inline embed on /consulting.
(function () {
  var calLink = document.body && document.body.getAttribute("data-cal-link");
  if (!calLink || !window.Cal) return; // no Cal -> href fallback (opens cal.com)

  // Turn every link to cal.com into a popup overlay instead of a new tab.
  document.querySelectorAll('a[href*="cal.com/"]').forEach(function (a) {
    a.setAttribute("data-cal-link", calLink);
    a.setAttribute("data-cal-config", '{"layout":"month_view"}');
    a.removeAttribute("target");
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
