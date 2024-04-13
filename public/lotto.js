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
                const numberDiv = document.createElement('div');
                const label = document.createElement('p');
                 
                numberDiv.className = 'flex font-bold rounded-full text-center align-middle text-white w-10 h-10 justify-center m-auto';
                numberDiv.style.borderColor = '#ffffff'
                numberDiv.style.borderWidth = '2px';
    
                label.className = 'align-middle m-auto justify-center';
                label.style.fontSize = '1rem';
                
                label.innerHTML = num;
                numberDiv.appendChild(label);
                numberSelectedDiv.appendChild(numberDiv);
                
            });                
        });
}

window.onload = ldcall;
