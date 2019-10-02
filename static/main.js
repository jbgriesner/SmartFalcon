const url = '/submit';
const form = document.querySelector('form');
const odds = document.getElementById("odds");
const Http = new XMLHttpRequest();

document.getElementById('empire').addEventListener('change', getFile)

let json;

function getFile(event) {
        const input = event.target
        if ('files' in input && input.files.length > 0) {
                let file = input.files[0];

                readFileContent(file).then(content => {
                        json = JSON.parse(content);
                }).catch(error => console.log(error))
        }
}

function readFileContent(file) {
        const reader = new FileReader()
        return new Promise((resolve, reject) => {
                reader.onload = event => resolve(event.target.result)
                reader.onerror = error => reject(error)
                reader.readAsText(file)
        })
}

form.addEventListener('submit', e => {
        e.preventDefault()
        alert('json = \n' + JSON.stringify(json));

}, false);

