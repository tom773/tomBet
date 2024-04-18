var options = [
    0, 32, 15, 19, 4, 21, 2, 25,
    17, 34, 6, 27, 13, 36, 11, 30,
    8, 23, 10, 5, 24, 16, 33, 1,
    20, 14, 31, 9, 22, 18, 29, 7,
    28, 12, 35, 3, 26
];

let hashMap = new Map<string, number>();
hashMap.set('zero', 0);
hashMap.set('one', 1);
hashMap.set('two', 2);
hashMap.set('three', 3);
hashMap.set('four', 4);
hashMap.set('five', 5);
hashMap.set('six', 6);
hashMap.set('seven', 7);
hashMap.set('eight', 8);
hashMap.set('nine', 9);
hashMap.set('ten', 10);
hashMap.set('eleven', 11);
hashMap.set('twelve', 12);
hashMap.set('thirteen', 13);
hashMap.set('fourteen', 14);
hashMap.set('fifteen', 15);
hashMap.set('sixteen', 16);
hashMap.set('seventeen', 17);
hashMap.set('eighteen', 18);
hashMap.set('nineteen', 19);
hashMap.set('twenty', 20);
hashMap.set('twentyone', 21);
hashMap.set('twentytwo', 22);
hashMap.set('twentythree', 23);
hashMap.set('twentyfour', 24);
hashMap.set('twentyfive', 25);
hashMap.set('twentysix', 26);
hashMap.set('twentyseven', 27);
hashMap.set('twentyeight', 28);
hashMap.set('twentynine', 29);    
hashMap.set('thirty', 30);
hashMap.set('thirtyone', 31);
hashMap.set('thirtytwo', 32);
hashMap.set('thirtythree', 33);
hashMap.set('thirtyfour', 34);
hashMap.set('thirtyfive', 35);
hashMap.set('thirtysix', 36);


function writeNum(number: number,  numbox: HTMLElement, last: HTMLElement) {

    numbox.innerHTML = number.toString();
    var lastlabel = document.createElement('h1');
    lastlabel.innerHTML = number.toString();
    last.appendChild(lastlabel);
}
var betsMap = new Map<number, number>();
async function declareWinner() {

    var winDiv = document.getElementById('win')!;
    winDiv.style.visibility = 'visible';

}
async function getw() {
    
    fetch('/api/spin') 
        .then(response => response.json())
        .then(data => {
            var winDiv = document.getElementById('win')!;
            winDiv.style.visibility = 'hidden';

            var number = data.num;

            var numbox = document.getElementById('numbox')!;
            var last = document.getElementById('last')!; 
            var wheel = document.getElementById('wheel')!;

            wheel.style.transition = 'none';
            wheel.style.transform = 'none';
            wheel.offsetHeight;

            var deg = (360 * 3) + (360-(singleRotation * (options.indexOf(number))));
            wheel.style.transition = "all 5s ease-out";
            wheel.style.transform = "rotate(" + deg + "deg)";
            
            var total = 0;
            if (betsMap != null) {
                for (var [key, value] of betsMap) {
                     total += value;
                     if (key == number) {
                        setTimeout(function(){declareWinner();}, 5500);
                     }
                }
            }
            setTimeout(function(){writeNum(number, numbox, last);}, 5500);
            
        });
}
function addBet(divNum: string){
    var betAmt = 100;
    if (betsMap.has(hashMap.get(divNum))){
        var existingBet =  document.getElementById(divNum).innerHTML!;
        var newBetAmt = parseInt(existingBet) + 100;
        betsMap.set(hashMap.get(divNum), newBetAmt);
        document.getElementById(divNum).innerHTML = newBetAmt.toString();
        return;
    }
    betsMap.set(hashMap.get(divNum), betAmt);
    var bet = document.getElementById(divNum)!;
    bet.innerHTML = betAmt.toString();
    bet.style.visibility = 'visible';
}

function clear(){
    console.log("Clearing");
    var bets = Array.from(document.querySelectorAll('.bet') as NodeListOf<HTMLElement>);
    for (var i = 0; i < bets.length; i++){
        bets[i].innerHTML = '';
        bets[i].style.visibility = 'hidden';
    }
}

var singleRotation = 360/37;

window.addEventListener('DOMContentLoaded', function() {
    var spinner = document.getElementById("spinbut")!;
    spinner.addEventListener("click", function(e) {
        e.preventDefault();
        getw();
    }, false);
    
    var clearbtn = document.getElementById("clear")!;
    clearbtn.addEventListener("click", function(e) {
        e.preventDefault();
        clear();
    }, false);
});
