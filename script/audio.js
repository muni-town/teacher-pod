var audio = document.getElementById("podcast-player");
auido.loop = false;
audio.addEventListener('ended', function() {
    alert("ended");
});