const mode = localStorage.getItem("mode");
console.log("TeacherPod: Current Mode: ", mode);
if (mode.toLowerCase == "dark") {
    document.documentElement.classList.add('dark')
} else {
    document.documentElement.classList.remove('dark')
}