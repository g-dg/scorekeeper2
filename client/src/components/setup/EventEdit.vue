<script lang="ts">
const defaultEvent: Event = {
  id: null,
  competition_id: "",
  name: "",
  description: "",
  enabled: true,
};
</script>

<script setup lang="ts">
import type { Competition } from "@/api/competitions";
import { EventsClient, type Event } from "@/api/events";
import clone from "@/helpers/clone";
import { computed, onMounted, ref, watch, type PropType } from "vue";

const props = defineProps({
  loading: {
    type: Number,
    default: 0,
  },
  event: {
    type: Object as PropType<Event>,
    default: () => clone(defaultEvent),
  },
  competitions: {
    type: Array as PropType<Competition[]>,
    required: true,
  },
});

const event = ref<Event>(clone(props.event));
watch(
  computed(() => props.event),
  () => (event.value = clone(props.event))
);
onMounted(() => (event.value = clone(props.event)));

const emit = defineEmits<{
  (e: "update"): void;
}>();

const selfLoading = ref(0);
const loading = computed(() => props.loading + selfLoading.value);

function validate() {
  if ((event.value.competition_id ?? "") == "") {
    alert("Competition is required");
    return false;
  }

  return true;
}

async function create() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await EventsClient.createEvent(event.value);

    event.value = clone(props.event);
  } catch (e) {
    console.error(e);
    alert("Error occurred creating event");
  }
  selfLoading.value--;
  emit("update");
}

async function update() {
  if (!validate()) return;

  selfLoading.value++;
  try {
    await EventsClient.updateEvent(event.value.id!, event.value);
  } catch (e) {
    console.error(e);
    alert("Error occurred updating event");
  }
  selfLoading.value--;
  emit("update");
}

async function remove() {
  if (!confirm(`Really delete event "${event.value.name}"?`)) {
    return;
  }

  selfLoading.value++;
  try {
    await EventsClient.deleteEvent(event.value.id!);
  } catch (e) {
    console.error(e);
    alert("Error occurred deleting event");
  }
  selfLoading.value--;
  emit("update");
}
</script>

<template>
  <form v-if="loading == 0" @submit.prevent>
    <label> Competition: </label>
    <select v-model="event.competition_id">
      <option :value="null"></option>
      <option v-for="competition in competitions" :value="competition.id">
        {{ competition.name }}
      </option>
    </select>

    <label> Name: </label>
    <input v-model="event.name" type="text" />

    <label> Description: </label>
    <textarea v-model="event.description"></textarea>

    <label> Enabled: </label>
    <input v-model="event.enabled" type="checkbox" />

    <button v-if="event.id == null" @click="create" type="submit">
      Create
    </button>
    <button v-if="event.id != null" @click="update" type="submit">
      Update
    </button>
    <button v-if="event.id != null" @click="remove">Delete</button>

    <template v-if="event.id != null">
      ID: <code>{{ event.id }}</code>
      <br />
    </template>
  </form>
</template>

<style lang="scss" scoped></style>
