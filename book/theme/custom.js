// Make the book title clickable and link to the same page
document.addEventListener('DOMContentLoaded', function() {
    const menuTitle = document.querySelector('.menu-title');
    if (menuTitle) {
        // Wrap the title text in a link to the current page
        const titleText = menuTitle.textContent;
        menuTitle.innerHTML = '<a href="' + window.location.pathname + '">' + titleText + '</a>';
    }
    
    // Start with sidebar collapsed
    const html = document.querySelector('html');
    if (html && !localStorage.getItem('mdbook-sidebar')) {
        // Only hide on first visit or if user hasn't set a preference
        html.classList.remove('sidebar-visible');
    }
});
