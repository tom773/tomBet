function generateNumberSelection(){
    for (let i = 1; i <= 22; i++) {
        const numberSelectionDiv = document.getElementById('numberSelection1');
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.id = `number${i}`;
        checkbox.value = i;
        const label = document.createElement('label');
        label.htmlFor = `number${i}`;
        label.appendChild(document.createTextNode(i));
        numberSelectionDiv.appendChild(checkbox);
        numberSelectionDiv.appendChild(label);
    }
    // Put this in an if statement at some point
    for (let i = 23; i <= 44; i++) {
        const numberSelectionDiv = document.getElementById('numberSelection2');
        const checkbox = document.createElement('input');
        checkbox.type = 'checkbox';
        checkbox.id = `number${i}`;
        checkbox.value = i;
        const label = document.createElement('label');
        label.htmlFor = `number${i}`;
        label.appendChild(document.createTextNode(i));
        numberSelectionDiv.appendChild(checkbox);
        numberSelectionDiv.appendChild(label);
    }

}

function submitTicket(){
    const selectedNumbers = [];
    const checkboxes = document.querySelectorAll('input[type=checkbox]');
    checkboxes.forEach(checkbox => {
        if (checkbox.checked) {
            selectedNumbers.push(checkbox.value);
        }
    });
    let fdata = new FormData();
    fdata.append('numbers', selectedNumbers);
    fetch('/api/select-numbers', {
        method: 'POST', 
        body: (fdata),
    });
    

    // To Do: Send selectedNumbers to server
    // then store it, and display it on the page in place of the selection boxes
    // Also, add a button to allow the user to select new numbers
}

function ldcall(){
    
    getTicket();
    generateNumberSelection();
}

function getTicket(){
    
    fetch('/api/getnums')
        .then(response => response.json())
        .then(data => {
            data.nums.forEach(num => {
                const numberSelectedDiv = document.getElementById('ticketNumbers_');
                const label = document.createElement('p');
                label.className = 'flex font-bold rounded-full text-center bg-green-400 align-middle text-white w-10 h-10 justify-center'
                label.appendChild(document.createTextNode(num));
                numberSelectedDiv.appendChild(label);
            });                
        });

}

window.onload = ldcall;
