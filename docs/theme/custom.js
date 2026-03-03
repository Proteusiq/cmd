// cmd Documentation - Enhanced JavaScript

document.addEventListener('DOMContentLoaded', function() {
    
    // ============================================
    // Copy Button Enhancement
    // ============================================
    document.querySelectorAll('.clip-button').forEach(function(button) {
        button.addEventListener('click', function() {
            const originalHTML = this.innerHTML;
            
            // Success animation
            this.innerHTML = '<span style="display:inline-flex;align-items:center;gap:4px;">Copied!</span>';
            this.style.background = 'linear-gradient(135deg, #10b981 0%, #059669 100%)';
            this.style.transform = 'scale(1.1)';
            
            setTimeout(() => {
                this.innerHTML = originalHTML;
                this.style.background = '';
                this.style.transform = '';
            }, 1500);
        });
    });

    // ============================================
    // Smooth Scrolling for Anchor Links
    // ============================================
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const targetId = this.getAttribute('href');
            if (targetId === '#') return;
            
            const target = document.querySelector(targetId);
            if (target) {
                e.preventDefault();
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
                
                // Update URL without jumping
                history.pushState(null, null, targetId);
            }
        });
    });

    // ============================================
    // External Links - Open in New Tab
    // ============================================
    document.querySelectorAll('a[href^="http"]').forEach(link => {
        if (!link.querySelector('img') && !link.closest('.sidebar')) {
            link.setAttribute('target', '_blank');
            link.setAttribute('rel', 'noopener noreferrer');
            
            // Add subtle external indicator
            if (!link.innerHTML.includes('↗')) {
                link.innerHTML += '<span style="font-size:0.75em;margin-left:2px;opacity:0.7;">↗</span>';
            }
        }
    });

    // ============================================
    // Heading Anchor Links on Hover
    // ============================================
    document.querySelectorAll('.content h2[id], .content h3[id], .content h4[id]').forEach(heading => {
        const link = document.createElement('a');
        link.href = '#' + heading.id;
        link.className = 'header-anchor';
        link.innerHTML = '#';
        link.style.cssText = `
            opacity: 0;
            margin-left: 0.5rem;
            color: var(--accent-color);
            text-decoration: none;
            transition: opacity 0.2s ease;
            font-weight: normal;
        `;
        
        heading.appendChild(link);
        
        heading.addEventListener('mouseenter', () => link.style.opacity = '0.7');
        heading.addEventListener('mouseleave', () => link.style.opacity = '0');
        link.addEventListener('mouseenter', () => link.style.opacity = '1');
    });

    // ============================================
    // Code Block Language Labels
    // ============================================
    document.querySelectorAll('pre > code').forEach(codeBlock => {
        const classes = codeBlock.className.split(' ');
        const langClass = classes.find(c => c.startsWith('language-'));
        
        if (langClass) {
            const lang = langClass.replace('language-', '');
            const label = document.createElement('span');
            label.className = 'code-lang-label';
            label.textContent = lang;
            label.style.cssText = `
                position: absolute;
                top: 8px;
                right: 50px;
                font-size: 0.7rem;
                text-transform: uppercase;
                letter-spacing: 0.05em;
                padding: 2px 8px;
                border-radius: 4px;
                background: rgba(99, 102, 241, 0.2);
                color: var(--accent-color-light);
                font-family: system-ui, sans-serif;
                font-weight: 600;
            `;
            
            const pre = codeBlock.parentElement;
            pre.style.position = 'relative';
            pre.appendChild(label);
        }
    });

    // ============================================
    // Active Section Highlighting in Sidebar
    // ============================================
    const observerOptions = {
        root: null,
        rootMargin: '-20% 0px -60% 0px',
        threshold: 0
    };

    const headingObserver = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            const id = entry.target.getAttribute('id');
            const tocLink = document.querySelector(`.sidebar a[href$="#${id}"]`);
            
            if (tocLink) {
                if (entry.isIntersecting) {
                    // Remove active from all
                    document.querySelectorAll('.sidebar a').forEach(a => {
                        a.style.fontWeight = '';
                        a.style.color = '';
                    });
                    // Add active to current
                    tocLink.style.fontWeight = '600';
                    tocLink.style.color = 'var(--accent-color)';
                }
            }
        });
    }, observerOptions);

    document.querySelectorAll('.content h2[id], .content h3[id]').forEach(heading => {
        headingObserver.observe(heading);
    });

    // ============================================
    // Keyboard Shortcuts
    // ============================================
    document.addEventListener('keydown', function(e) {
        // Press '/' to focus search
        if (e.key === '/' && !e.ctrlKey && !e.metaKey) {
            const searchBar = document.getElementById('searchbar');
            if (searchBar && document.activeElement !== searchBar) {
                e.preventDefault();
                searchBar.focus();
            }
        }
        
        // Press 'Escape' to blur search
        if (e.key === 'Escape') {
            const searchBar = document.getElementById('searchbar');
            if (searchBar && document.activeElement === searchBar) {
                searchBar.blur();
            }
        }
    });

    // ============================================
    // Search Placeholder Enhancement
    // ============================================
    const searchBar = document.getElementById('searchbar');
    if (searchBar) {
        searchBar.placeholder = 'Search docs... (press /)';
    }

    // ============================================
    // Table Row Highlighting
    // ============================================
    document.querySelectorAll('table tbody tr').forEach(row => {
        row.addEventListener('mouseenter', function() {
            this.style.transition = 'all 0.2s ease';
        });
    });

    // ============================================
    // Scroll Progress Indicator
    // ============================================
    const progressBar = document.createElement('div');
    progressBar.id = 'scroll-progress';
    progressBar.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        height: 3px;
        background: linear-gradient(90deg, #6366f1 0%, #8b5cf6 50%, #a855f7 100%);
        z-index: 9999;
        transition: width 0.1s ease-out;
        width: 0%;
    `;
    document.body.appendChild(progressBar);

    window.addEventListener('scroll', () => {
        const scrollTop = window.scrollY;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const scrollPercent = (scrollTop / docHeight) * 100;
        progressBar.style.width = scrollPercent + '%';
    });

    // ============================================
    // Back to Top Button
    // ============================================
    const backToTop = document.createElement('button');
    backToTop.innerHTML = '↑';
    backToTop.id = 'back-to-top';
    backToTop.title = 'Back to top';
    backToTop.style.cssText = `
        position: fixed;
        bottom: 30px;
        right: 30px;
        width: 45px;
        height: 45px;
        border-radius: 50%;
        background: linear-gradient(135deg, #6366f1 0%, #8b5cf6 100%);
        color: white;
        border: none;
        cursor: pointer;
        font-size: 1.25rem;
        font-weight: bold;
        box-shadow: 0 4px 15px rgba(99, 102, 241, 0.4);
        opacity: 0;
        visibility: hidden;
        transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
        z-index: 9998;
        display: flex;
        align-items: center;
        justify-content: center;
    `;
    document.body.appendChild(backToTop);

    backToTop.addEventListener('click', () => {
        window.scrollTo({ top: 0, behavior: 'smooth' });
    });

    backToTop.addEventListener('mouseenter', () => {
        backToTop.style.transform = 'scale(1.1)';
        backToTop.style.boxShadow = '0 6px 20px rgba(99, 102, 241, 0.5)';
    });

    backToTop.addEventListener('mouseleave', () => {
        backToTop.style.transform = 'scale(1)';
        backToTop.style.boxShadow = '0 4px 15px rgba(99, 102, 241, 0.4)';
    });

    window.addEventListener('scroll', () => {
        if (window.scrollY > 300) {
            backToTop.style.opacity = '1';
            backToTop.style.visibility = 'visible';
        } else {
            backToTop.style.opacity = '0';
            backToTop.style.visibility = 'hidden';
        }
    });

    // ============================================
    // Image Zoom on Click (if any images)
    // ============================================
    document.querySelectorAll('.content img').forEach(img => {
        img.style.cursor = 'zoom-in';
        img.style.transition = 'transform 0.3s ease';
        img.style.borderRadius = '8px';
        
        img.addEventListener('click', function() {
            if (this.style.transform === 'scale(1.5)') {
                this.style.transform = 'scale(1)';
                this.style.cursor = 'zoom-in';
                this.style.zIndex = '';
                this.style.position = '';
            } else {
                this.style.transform = 'scale(1.5)';
                this.style.cursor = 'zoom-out';
                this.style.zIndex = '100';
                this.style.position = 'relative';
            }
        });
    });

    // ============================================
    // Console Easter Egg
    // ============================================
    console.log('%c cmd ', 
        'background: linear-gradient(135deg, #6366f1, #a855f7); color: white; font-size: 24px; font-weight: bold; padding: 10px 20px; border-radius: 8px;'
    );
    console.log('%cYour words become commands.', 
        'color: #6366f1; font-size: 14px; font-style: italic;'
    );
});
