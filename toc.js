// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">User Guide</li><li class="chapter-item expanded "><a href="motivation.html"><strong aria-hidden="true">1.</strong> Motivation</a></li><li class="chapter-item expanded "><a href="prior_art.html"><strong aria-hidden="true">2.</strong> Prior Art</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">3.</strong> Installation</a></li><li class="chapter-item expanded "><a href="usage/usage.html"><strong aria-hidden="true">4.</strong> Usage</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="usage/fwd.html"><strong aria-hidden="true">4.1.</strong> Forward Mode</a></li><li class="chapter-item expanded "><a href="usage/rev.html"><strong aria-hidden="true">4.2.</strong> Reverse Mode</a></li><li class="chapter-item expanded "><a href="usage/higher.html"><strong aria-hidden="true">4.3.</strong> Higher Order Derivatives</a></li><li class="chapter-item expanded "><a href="usage/python.html"><strong aria-hidden="true">4.4.</strong> Python Integration</a></li></ol></li><li class="chapter-item expanded "><a href="limitations.html"><strong aria-hidden="true">5.</strong> Current limitations &amp; future work</a></li><li class="chapter-item expanded "><a href="ecosystem.html"><strong aria-hidden="true">6.</strong> History and ecosystem</a></li><li class="chapter-item expanded "><a href="Debugging.html"><strong aria-hidden="true">7.</strong> How to Debug</a></li><li class="chapter-item expanded affix "><li class="part-title">Reference Guide</li><li class="chapter-item expanded "><a href="other_Frontends.html"><strong aria-hidden="true">8.</strong> Other Enzyme frontends</a></li><li class="chapter-item expanded "><a href="fwd.html"><strong aria-hidden="true">9.</strong> Forward Mode</a></li><li class="chapter-item expanded "><a href="rev.html"><strong aria-hidden="true">10.</strong> Reverse Mode</a></li><li class="chapter-item expanded "><a href="user_design.html"><strong aria-hidden="true">11.</strong> User facing design</a></li><li class="chapter-item expanded "><a href="rustc_design.html"><strong aria-hidden="true">12.</strong> rustc internal design</a></li><li class="chapter-item expanded "><a href="unsafe.html"><strong aria-hidden="true">13.</strong> Unsafe Interface</a></li><li class="chapter-item expanded "><a href="acknowledgments.html"><strong aria-hidden="true">14.</strong> Acknowledgments</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
