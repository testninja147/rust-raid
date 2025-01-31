const todo = {
  async list() {
    try {
      let list = await fetch("/api/todo/");
      return await list.json()
    } catch (err) {
      console.log("Failed to fetch")
      return [];
    }
  },
  async create() {
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
  async delete(id) {
    return fetch(`/api/todo/${id}/`, {
      method: 'DELETE',
      headers: {
        'Content-Type': "application/json"
      }
    }).then(res => {
      window.location.reload();
    })
  },
  async update(id, body) {
    return fetch(`/api/todo/${id}/`, {
      method: 'PATCH',
      body: JSON.stringify(body),
      headers: {
        'Content-Type': "application/json"
      }
    }).then(res => {
      window.location.reload();
    })
  },
  async check(id, checked) {
    return this.update(id, { checked })
  }
}



document.addEventListener('DOMContentLoaded', async () => {
  const container = document.querySelector('div#todo-list');
  // move checked to the bottom
  const elements = (await todo.list()).sort((a, b) => a.checked - b.checked).map(d => {
    return `
    <div class="todo-item group flex flex-col bg-slate-500/30 gap-2 p-4" data-target="${d.id}">
      <span class="flex items-center w-full justify-between h-16 gap-2  p-2 px-4 ">
        <input class="todo-check" type="checkbox" ${d.checked ? "checked" : ""} data-target="${d.id}">
        <span class="font-bold flex grow${d.checked ? ' line-through' : ''}">${d.title}</span>
        <span class="hidden group-hover:flex gap-2">
          <button class="todo-edit bg-blue-900 text-white p-2 rounded-md hover:brightness-120" data-target="${d.id}"data-action="edit">
            Edit
          </button>
          <button class="todo-delete bg-red-800 text-white p-2 rounded-md hover:brightness-120" data-target="${d.id}" data-action="delete">
            Delete
          </button>
        </span>
      </span>
      <div class="bg-white/20 p-2 px-4">
      ${d.content}
      </div>
    </div>
    `
  });
  container.innerHTML = elements.join('');

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

  document.querySelectorAll("input.todo-check").forEach((checkbox) => {
    checkbox.addEventListener('change', (event) => {
      const todoId = event.target.getAttribute('data-target');
      todo.check(todoId, event.target.checked);
    })
  })
});
