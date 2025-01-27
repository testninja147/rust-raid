const todo = {
    list: async () => {
        try {
            let list = await fetch("/api/todo/");
            return await list.json()
        } catch (err) {
            console.log("Failed to fetch")
            return [];
        }
    },
    create: async () => {
        await fetch("/api/todo/", {method: 'POST', body})
    }
}



document.addEventListener('DOMContentLoaded', async () => {
    const sidebar = document.querySelector('div.sidebar');
    const elements = (await todo.list()).map(d => {
        return `<div class="todo${d.checked ? ' checked' : ''}">${d.title}</div>`
    });
    sidebar.innerHTML = elements.join('');

    // add new todo event listener
    document.querySelector('button#add-todo').addEventListener('click', () => {
        document.querySelector("#add-modal").classList.toggle('hidden');
    })

    document.querySelector("#submit-todo").addEventListener('click', (event) => {
        event.preventDefault();
        const title = document.querySelector("input#title").value;
        const content = document.querySelector("textarea#content").value;
        fetch("/api/todo/", {
            method: 'POST',
            body: {
                title,
                content,
            },
        })
    });

    document.querySelector("#cancel-todo").addEventListener('click', (event) => {
        document.querySelector("input#title").value = "";
        document.querySelector("textarea#content").value="";
        document.querySelector("#add-modal").classList.add('hidden');
    });

});
