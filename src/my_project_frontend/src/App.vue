<script setup>
	import { ref } from "vue";
	import { my_project_backend } from "declarations/my_project_backend/index";
	let articles = ref([]);

    let currentTag = ref("");
    let addedTags = ref([]);

	function makeArticle(article) {
		return {
			...article,
            date: new Date(parseInt(article.date / BigInt(1_000_000))).toLocaleDateString(),
		};
	}

	async function getBlogs() {
		articles.value = (await my_project_backend.get_articles()).map(makeArticle);
	}

	async function handleSubmit(e) {
		e.preventDefault();

		const target = e.target;
		const form = target.closest("form");
		const formData = new FormData(form);

		const response = await my_project_backend.add_article(
			formData.get("title"),
			formData.get("author"),
			addedTags,
			formData.get("content")
		);

        if (response.Err) {
            alert(response.Err);
            return;
        }

		articles.value = [...articles.value, makeArticle(response.Ok)];
	}

    function handleTagChange (e) {
        if (e.key !== ",") return;
        e.preventDefault && e.preventDefault();
        addedTags.value =  [...addedTags.value, ...e.target.value.split(",")];
        currentTag.value = "";
    }

    function deleteTag (index) {
        addedTags.value = addedTags.value.filter((_, i) => i !== index);
    }

	getBlogs();
</script>

<template>
	<main>
		<form action="#" @submit="handleSubmit">
			<div class="group" id="title-group">
				<label for="title">Title</label>
				<input id="title" name="title" type="text" placeholder="ex. The giant lottery" />
			</div>
			<div class="group" id="author-group">
				<label for="author">by</label>
				<input id="author" name="author" type="text" placeholder="ex. Jack Smith"/>
			</div>
			<div class="group">
				<label for="content">Article text:</label>
				<textarea name="content" id="content" cols="30" rows="10"></textarea>
			</div>
            <div id="tags">
                <label for="tags">Tags</label>
                <input id="tags" name="tags" type="text" :value=currentTag @keydown="handleTagChange" />
                <ul id="selected-tags">
                    <li v-for="(tag, index) in addedTags" :key="index" class="tag">{{ tag }} <button @click="() => deleteTag(index)">x</button></li>
                </ul>
            </div>
			<button type="submit">Add</button>
		</form>
        {{ articles }}
	</main>
</template>
