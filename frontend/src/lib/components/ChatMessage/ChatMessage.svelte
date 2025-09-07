<!-- Chat message component that handles both file paths and base64 audio -->
<script lang="ts">
	import { revokeAudioURL } from '$lib/utils/audio-utils';
	import { isBase64Audio, convertTestAudioToUrl } from '$lib/services/chat-service';
	import { onDestroy } from 'svelte';

	export let message: {
		text: string[];
		audio: string[]; // File paths from chat history OR base64 from test endpoints
		images: string[];
	};

	// Store object URLs for cleanup (only needed for base64 audio)
	let objectUrls: string[] = [];
	let audioUrls: string[] = [];

	// Process audio data - convert base64 to URLs, keep file paths as-is
	$: {
		// Clean up previous object URLs
		objectUrls.forEach((url) => revokeAudioURL(url));
		objectUrls = [];

		// Process each audio item
		audioUrls = message.audio.map((audioData) => {
			if (isBase64Audio(audioData)) {
				// It's base64 from a test endpoint - convert to object URL
				const objectUrl = convertTestAudioToUrl(audioData);
				objectUrls.push(objectUrl); // Track for cleanup
				return objectUrl;
			} else {
				// It's a file path from chat history - use as-is
				return audioData;
			}
		});
	}

	// Clean up object URLs when component is destroyed
	onDestroy(() => {
		objectUrls.forEach((url) => revokeAudioURL(url));
	});
</script>

<div class="message">
	<!-- Text content -->
	{#each message.text as textLine}
		<div class="message-text">
			{textLine}
		</div>
	{/each}

	<!-- Audio content -->
	{#if audioUrls.length > 0}
		<div class="message-audio">
			{#each audioUrls as audioUrl, index}
				{@const isBase64Source = isBase64Audio(message.audio[index])}
				<div class="audio-item">
					{#if isBase64Source}
						<small class="audio-label">Generated Audio (Test)</small>
					{:else}
						<small class="audio-label">Audio File</small>
					{/if}
					<audio controls>
						<source src={audioUrl} type={isBase64Source ? 'audio/mp3' : ''} />
						{#if isBase64Source}
							<!-- Fallback for mp3 -->
							Audio format not supported by your browser
						{:else}
							<!-- Standard fallback -->
							Your browser does not support the audio element
						{/if}
					</audio>
				</div>
			{/each}
		</div>
	{/if}

	<!-- Image content -->
	{#if message.images.length > 0}
		<div class="message-images">
			{#each message.images as imageFile}
				<img src={imageFile} alt="Message attachment" />
			{/each}
		</div>
	{/if}
</div>

<style>
	.message {
		margin-bottom: 1rem;
		padding: 1rem;
		background: var(--color-surface-100);
		border-radius: 8px;
	}

	.message-text {
		margin-bottom: 0.5rem;
		line-height: 1.5;
	}

	.message-audio {
		margin-top: 0.5rem;
	}

	.audio-item {
		margin-bottom: 0.5rem;
	}

	.audio-label {
		display: block;
		color: var(--color-surface-600);
		margin-bottom: 0.25rem;
		font-size: 0.75rem;
		font-weight: 500;
		text-transform: uppercase;
		letter-spacing: 0.05em;
	}

	.message-audio audio {
		display: block;
		width: 100%;
		max-width: 400px;
	}

	.message-images {
		margin-top: 0.5rem;
	}

	.message-images img {
		max-width: 100%;
		height: auto;
		border-radius: 4px;
		margin-bottom: 0.5rem;
	}
</style>
