var elem = document.documentElement;
if (elem.requestFullscreen) {
    elem.requestFullscreen()
        .then(() => { })
        .catch((err) => {
            alert(
                `An error occurred while trying to switch into fullscreen mode: ${err.message} (${err.name})`,
            );
        });
} else if (elem.mozRequestFullScreen) { /* Firefox */
    elem.mozRequestFullScreen()
        .then(() => { })
        .catch((err) => {
            alert(
                `An error occurred while trying to switch into fullscreen mode: ${err.message} (${err.name})`,
            );
        });
} else if (elem.webkitRequestFullscreen) { /* Chrome, Safari & Opera */
    elem.webkitRequestFullscreen()
        .then(() => { })
        .catch((err) => {
            alert(
                `An error occurred while trying to switch into fullscreen mode: ${err.message} (${err.name})`,
            );
        });
} else if (elem.msRequestFullscreen) { /* IE/Edge */
    elem.msRequestFullscreen()
        .then(() => { })
        .catch((err) => {
            alert(
                `An error occurred while trying to switch into fullscreen mode: ${err.message} (${err.name})`,
            );
        });
}
