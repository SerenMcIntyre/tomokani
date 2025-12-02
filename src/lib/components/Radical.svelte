<script lang="ts">
    import type { Subject } from "$lib/models/subject";
    interface Props {
        subject?: Subject;
        complete: (updatedSubject: Subject) => void;
    }
    let { subject = $bindable(), complete }: Props = $props();
    if (!subject) {
        throw new Error("Subject is required");
    }

    let img = $derived(subject.data.character_images?.find((img => img.content_type === "image/svg+xml")));
    let character = $derived(subject.data.characters?.at(0))
    let name = $state("");

    const onsubmit = (event: Event) => {
        event.preventDefault();
        if (subject.data.meanings.some(m => m.meaning.toLowerCase() === name.toLowerCase())) {
            complete(subject);
        } else {
            subject.review_data = {
                incorrect_meaning_answers: (subject.review_data?.incorrect_meaning_answers ?? 0) + 1,
                incorrect_reading_answers: 0,
            };
        }
    };
</script>

<form class="border border-teal-700 bg-teal-300 rounded p-4 shadow-md"
    {onsubmit}>
    <h2 class="text-2xl font-semibold mb-4">Radical</h2>
    {#if img}
        <img src={img.url} alt={character ?? "Unknown"} class="mb-4 w-full bg-white rounded-xl h-20 object-contain" />
    {/if}

    <label class="block text-lg font-medium mb-2" for={subject.id+"-name"}>Name:</label>
    <input class="block bg-white rounded-xl py-2 px-4 w-full" type="text" id={subject.id+"-name"} bind:value={name} placeholder="Enter radical name" />

    <button type="submit" disabled={!name} class="mt-4 w-full px-4 py-2 bg-teal-700 text-white rounded hover:bg-teal-800">
        Submit
    </button>
</form>