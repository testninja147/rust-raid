const todo = {
    list: async () => {
        try {
            let list = await fetch("/api/todo/");
            return await list.json()
        } catch (err) {
            console.log("Failed to fetch")
            return [];
        }
    }
}



document.addEventListener('DOMContentLoaded', async () => {
    const sidebar = document.querySelector('div.sidebar');
    const elements = (await todo.list()).map(d => {
        return `<div class="todo${d.checked?' checked':''}">${d.title}</div>`
    });
    sidebar.innerHTML = elements.join('');
})
