var options = [
    0, 32, 15, 19, 4, 21, 2, 25,
    17, 34, 6, 27, 13, 36, 11, 30,
    8, 23, 10, 5, 24, 16, 33, 1,
    20, 14, 31, 9, 22, 18, 29, 7,
    28, 12, 35, 3, 26
];

function writeNum(number,  numbox, last) {

    numbox.innerHTML = number;
    var lastlabel = document.createElement('h1');
    lastlabel.innerHTML = number;
    last.appendChild(lastlabel);
}
async function getw() {
    
    fetch('/api/spin') 
        .then(response => response.json())
        .then(data => {
            var number = data.num;

            var numbox = document.getElementById('numbox');
            var last = document.getElementById('last'); 
            var wheel = document.getElementById('wheel');

            wheel.style.transition = 'none';
            wheel.style.transform = 'none';
            wheel.offsetHeight;

            var deg = (360 * 3) + (360-(singleRotation * (options.indexOf(number))));
            wheel.style.transition = "all 5s ease-out";
            wheel.style.transform = "rotate(" + deg + "deg)";
            
            setTimeout(function(){writeNum(number, numbox, last);}, 5500);
            
        });
}

var singleRotation = 360/37;

window.addEventListener('DOMContentLoaded', function() {
    var spinner = document.getElementById("spinbut");
    spinner.addEventListener("click", function(e) {
        e.preventDefault();
        getw();
    }, false);
});
