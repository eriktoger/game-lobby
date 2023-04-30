import styled from "styled-components";

function getShortName(full_name = "") {
  if (full_name.includes(" ")) {
    const names = full_name.split(" ");
    return `${names[0].charAt(0)}${names[1].charAt(0)}`.toUpperCase();
  }
  return `${full_name.slice(0, 2)}`.toUpperCase();
}

const StyledAvatar = styled.div`
  border-radius: 10px;
  height: 30px;
  width: 30px;
  text-align: center;
  line-height: 30px;
`;
export default function Avatar({
  children,
  color = "",
}: {
  children: string;
  color?: string;
}) {
  return (
    <StyledAvatar style={{ backgroundColor: color }}>
      <span>{getShortName(children)}</span>
    </StyledAvatar>
  );
}
