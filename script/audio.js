var audio = document.getElementById("podcast-player");
audio.loop = false;
audio.addEventListener('ended', function() {
    alert("ended");
});