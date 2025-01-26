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



document.addEventListener('DOMContentLoaded', () => {
    console.log("Document is ready");
})
