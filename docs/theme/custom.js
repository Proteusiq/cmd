// cmd Documentation - Custom JavaScript

// Add copy feedback animation
document.addEventListener('DOMContentLoaded', function() {
    // Enhance code copy buttons
    document.querySelectorAll('.clip-button').forEach(function(button) {
        button.addEventListener('click', function() {
            const originalText = this.innerHTML;
            this.innerHTML = '✓';
            this.style.background = '#10b981';
            setTimeout(() => {
                this.innerHTML = originalText;
                this.style.background = '';
            }, 1500);
        });
    });

    // Add smooth scrolling for anchor links
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                e.preventDefault();
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });

    // Add external link icons
    document.querySelectorAll('a[href^="http"]').forEach(link => {
        if (!link.querySelector('img')) {
            link.setAttribute('target', '_blank');
            link.setAttribute('rel', 'noopener noreferrer');
        }
    });

    // Highlight current section in TOC
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            const id = entry.target.getAttribute('id');
            const tocLink = document.querySelector(`.pagetoc a[href="#${id}"]`);
            if (tocLink) {
                if (entry.isIntersecting) {
                    tocLink.classList.add('active');
                } else {
                    tocLink.classList.remove('active');
                }
            }
        });
    }, { threshold: 0.5 });

    document.querySelectorAll('h2[id], h3[id]').forEach(heading => {
        observer.observe(heading);
    });
});
