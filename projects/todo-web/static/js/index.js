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
        const title = document.querySelector("input#title").value;
        const content = document.querySelector("textarea#content").value;
        document.querySelector("#submit-todo").innerHTML = "Loading..."
        return fetch("/api/todo/", {
            method: 'POST',
            headers: {
                'Content-Type': "application/json"
            },
            body: JSON.stringify({
                title,
                content,
            },
            ),
        }).then(res => {
            window.location.reload();
        })
    },
    delete: async (id) => {
        return fetch(`/api/todo/${id}/`, {
            method: 'DELETE',
            headers: {
                'Content-Type': "application/json"
            }
        }).then(res => {
            window.location.reload();
        })
    }
}



document.addEventListener('DOMContentLoaded', async () => {
    const sidebar = document.querySelector('div.sidebar');
    const elements = (await todo.list()).map(d => {
        return `
        <div class="group flex bg-slate-500/30 p-2 px-4 items-center justify-between h-16">
        <span class="flex grow ${d.checked ? ' checked' : ''}">${d.title}</span>
        <span class="hidden group-hover:flex gap-2">
        <button class="todo-edit bg-slate-800 text-white p-2 rounded-md" data-target="${d.id}" data-action="edit">Edit</button>
        <button class="todo-delete bg-red-800 text-white p-2 rounded-md" data-target="${d.id}" data-action="delete">Delete</button>
        </span>
        </div>
        `
    });
    sidebar.innerHTML = elements.join('');

    // add new todo event listener
    document.querySelector('button#add-todo').addEventListener('click', () => {
        document.querySelector("#add-modal").classList.toggle('hidden');
    })

    document.querySelector("#submit-todo").addEventListener('click', (event) => {
        event.preventDefault();
        todo.create();
    });

    document.querySelector("#cancel-todo").addEventListener('click', (event) => {
        document.querySelector("input#title").value = "";
        document.querySelector("textarea#content").value = "";
        document.querySelector("#add-modal").classList.add('hidden');
    });

    document.querySelectorAll("button.todo-delete").forEach((btn) => {
        btn.addEventListener('click', (event) => {
            const todoId = event.target.getAttribute('data-target');
            todo.delete(todoId);
        })
    })
});
