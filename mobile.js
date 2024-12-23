var elem = document.documentElement;
if (elem.requestFullscreen) {
    elem.requestFullscreen();
} else if (elem.mozRequestFullScreen) { /* Firefox */
    elem.mozRequestFullScreen();
} else if (elem.webkitRequestFullscreen) { /* Chrome, Safari & Opera */
    elem.webkitRequestFullscreen();
} else if (elem.msRequestFullscreen) { /* IE/Edge */
    elem.msRequestFullscreen();
}

// document.addEventListener('touchmove', function(event) {
//     event.preventDefault();
// }, { passive: false });

// window.addEventListener('scroll', function(event) {
//     event.preventDefault();
//     window.scrollTo(0, 0);
// });

// // Prevent default touch behavior
// document.addEventListener('touchstart', function(e) {
//     e.preventDefault();
// }, { passive: false });

// // Prevent zooming
// document.addEventListener('gesturestart', function(e) {
//     e.preventDefault();
// });
