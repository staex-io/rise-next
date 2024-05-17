<template>
    <h1>Video upload</h1>
    <form method="POST" enctype="multipart/form-data" action="/v1/video">
        <input type="file" name="video" required />
        <button>Upload</button>
    </form>
</template>
