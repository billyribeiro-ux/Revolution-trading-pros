export const load = async ({ params }: { params: { id: string } }) => {
	// In a real implementation, this would fetch from an API
	// Example: const response = await fetch(`/api/indicators/${params.id}`);
	// For now, we'll return the indicator ID to be used by the component
	return {
		indicatorId: params.id
	};
};
