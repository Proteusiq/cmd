// cmd Documentation - Terminal Theme JavaScript

document.addEventListener('DOMContentLoaded', function() {
    
    // ============================================
    // Terminal-style Copy Button
    // ============================================
    document.querySelectorAll('.clip-button').forEach(function(button) {
        button.addEventListener('click', function() {
            const originalHTML = this.innerHTML;
            
            this.innerHTML = '[ COPIED ]';
            this.style.background = '#00ff00';
            this.style.color = '#0d1117';
            this.style.borderColor = '#00ff00';
            
            setTimeout(() => {
                this.innerHTML = originalHTML;
                this.style.background = '';
                this.style.color = '';
                this.style.borderColor = '';
            }, 1500);
        });
    });

    // ============================================
    // Smooth Scrolling
    // ============================================
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            const targetId = this.getAttribute('href');
            if (targetId === '#') return;
            
            const target = document.querySelector(targetId);
            if (target) {
                e.preventDefault();
                target.scrollIntoView({ behavior: 'smooth', block: 'start' });
                history.pushState(null, null, targetId);
            }
        });
    });

    // ============================================
    // External Links
    // ============================================
    document.querySelectorAll('a[href^="http"]').forEach(link => {
        if (!link.querySelector('img') && !link.closest('.sidebar')) {
            link.setAttribute('target', '_blank');
            link.setAttribute('rel', 'noopener noreferrer');
        }
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
            label.textContent = lang.toUpperCase();
            label.style.cssText = `
                position: absolute;
                top: 7px;
                right: 45px;
                font-size: 0.65rem;
                text-transform: uppercase;
                letter-spacing: 0.1em;
                padding: 2px 6px;
                background: #21262d;
                color: #6272a4;
                font-family: 'JetBrains Mono', monospace;
                border: 1px solid #30363d;
                border-radius: 3px;
            `;
            
            const pre = codeBlock.parentElement;
            pre.style.position = 'relative';
            pre.appendChild(label);
        }
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
        
        // Press 'Escape' to blur
        if (e.key === 'Escape') {
            const searchBar = document.getElementById('searchbar');
            if (searchBar) searchBar.blur();
        }
    });

    // ============================================
    // Search Placeholder
    // ============================================
    const searchBar = document.getElementById('searchbar');
    if (searchBar) {
        searchBar.placeholder = '/ search...';
    }

    // ============================================
    // Terminal Scroll Progress
    // ============================================
    const progressBar = document.createElement('div');
    progressBar.style.cssText = `
        position: fixed;
        top: 0;
        left: 0;
        height: 2px;
        background: #00ff00;
        box-shadow: 0 0 10px rgba(0, 255, 0, 0.5);
        z-index: 9999;
        width: 0%;
        transition: width 0.1s ease-out;
    `;
    document.body.appendChild(progressBar);

    window.addEventListener('scroll', () => {
        const scrollTop = window.scrollY;
        const docHeight = document.documentElement.scrollHeight - window.innerHeight;
        const scrollPercent = docHeight > 0 ? (scrollTop / docHeight) * 100 : 0;
        progressBar.style.width = scrollPercent + '%';
    });

    // ============================================
    // Back to Top Button - Terminal Style
    // ============================================
    const backToTop = document.createElement('button');
    backToTop.innerHTML = '^';
    backToTop.title = '[TOP]';
    backToTop.style.cssText = `
        position: fixed;
        bottom: 25px;
        right: 25px;
        width: 40px;
        height: 40px;
        background: #0d1117;
        color: #00ff00;
        border: 1px solid #00ff00;
        cursor: pointer;
        font-size: 1.25rem;
        font-family: 'JetBrains Mono', monospace;
        font-weight: bold;
        opacity: 0;
        visibility: hidden;
        transition: all 0.2s ease;
        z-index: 9998;
    `;
    document.body.appendChild(backToTop);

    backToTop.addEventListener('click', () => {
        window.scrollTo({ top: 0, behavior: 'smooth' });
    });

    backToTop.addEventListener('mouseenter', () => {
        backToTop.style.background = '#00ff00';
        backToTop.style.color = '#0d1117';
        backToTop.style.boxShadow = '0 0 15px rgba(0, 255, 0, 0.5)';
    });

    backToTop.addEventListener('mouseleave', () => {
        backToTop.style.background = '#0d1117';
        backToTop.style.color = '#00ff00';
        backToTop.style.boxShadow = '';
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
    // Add prompt symbol to bash code blocks
    // ============================================
    document.querySelectorAll('pre > code.language-bash, pre > code.language-shell, pre > code.language-sh').forEach(codeBlock => {
        const lines = codeBlock.innerHTML.split('\n');
        const processedLines = lines.map(line => {
            // Skip empty lines and comments
            if (line.trim() === '' || line.trim().startsWith('#')) {
                return line;
            }
            // Skip lines that already have a prompt or are output
            if (line.trim().startsWith('$') || line.trim().startsWith('>') || line.trim().startsWith('→')) {
                return line;
            }
            return line;
        });
        codeBlock.innerHTML = processedLines.join('\n');
    });

    // ============================================
    // Console Easter Egg - Terminal Style
    // ============================================
    console.log('%c┌─────────────────────────────────────┐', 'color: #00ff00');
    console.log('%c│           cmd v0.5.2                │', 'color: #00ff00');
    console.log('%c│   Your words become commands.       │', 'color: #00ff00');
    console.log('%c└─────────────────────────────────────┘', 'color: #00ff00');
    console.log('%c> _', 'color: #00ff00');
});
