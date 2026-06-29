import { ICONS } from "@/lib/icons";

type ListIconProps = {
	name: string | null;
	className?: string;
};

export function ListIcon({ name, className }: ListIconProps) {
	if (!name) {
		return null;
	}

	const icon = ICONS[name as keyof typeof ICONS];

	if (!icon) {
		return null;
	}

	const Icon = icon.icon;

	return <Icon className={className} />;
}
