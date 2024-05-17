<template>
    <h1>Photo upload</h1>
    <form method="POST" enctype="multipart/form-data" action="/v1/photo">
        <input type="file" name="photo" required />
        <button>Upload</button>
    </form>
</template>
