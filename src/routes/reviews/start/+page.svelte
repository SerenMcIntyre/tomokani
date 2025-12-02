<script lang="ts">
  import { goto } from "$app/navigation";
  import Radical from "$lib/components/Radical.svelte";
  import type { Subject } from "$lib/models/subject";
  import { invoke } from "@tauri-apps/api/core";
  import { reviewSession } from "../review-session.svelte";
  import type { PageData } from "./$types";
  import * as v from "valibot";
  import { ReviewSchema } from "$lib/models/review";

  let { data }: { data: PageData } = $props();

  if (!reviewSession.reviewIds || reviewSession.reviewIds.length === 0) {
    goto("/reviews");
  }

  const complete = (updatedSubject: Subject) => {
    try {
      const reviewData = v.parse(ReviewSchema, {
        subject_id: updatedSubject.id,
        incorrect_meaning_answers:
          updatedSubject.review_data?.incorrect_meaning_answers ?? 0,
        incorrect_reading_answers:
          updatedSubject.review_data?.incorrect_reading_answers ?? 0,
        created_at: new Date().toISOString(),
      });
      alert("try submit review with data " + JSON.stringify({ review: reviewData }));

      invoke("submit_review", { review: { review: reviewData } })
        .then(() => {
          alert(`Review for subject ID ${updatedSubject.id} submitted.`);
          reviewSession.reviewIds = reviewSession.reviewIds.filter(
            (id) => id !== updatedSubject.id,
          );
        })
        .catch((error) => {
          console.error("Error submitting review:", error);
          alert(
            "Failed to submit review. Please try again. " +
              JSON.stringify(error),
          );
        });
    } catch (e) {
      console.error("Validation error:", e);
      alert(
        "Failed to submit review due to validation error. " + JSON.stringify(e),
      );
    }
  };
</script>

{#each reviewSession.reviewIds as id}
  {@const subject = data.subjects.find((s) => s.id === id)}
  <div class="m-4 p-4 border border-gray-300 shadow-sm rounded">
    <p>Reviewing subject ID: {id}</p>
    {#if subject?.object === "radical"}
      <Radical
        bind:subject={
          () => data.subjects.find((s) => s.id === id),
          (v) => {
            const index = data.subjects.findIndex((s) => s.id === id);
            if (index !== -1 && !!v) {
              data.subjects[index] = v;
            }
          }
        }
        {complete}
      />
    {:else}
      <p>Subject not found.</p>
    {/if}
  </div>
{/each}
