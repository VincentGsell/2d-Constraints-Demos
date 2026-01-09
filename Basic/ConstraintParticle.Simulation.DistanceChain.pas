unit ConstraintParticle.Simulation.DistanceChain;

interface

uses
  System.Types,
  System.UITypes,
  System.SysUtils,
  ConstraintParticle;

type
  TDistanceChainSimulation = class(TConstraintParticleSimulation)
  private
    FLinkDistance: Single;
    FUseFabrik: Boolean;
    FBallCollision: Boolean;
    FAnchorPos: TPointF;
    class function RandomColor: TAlphaColor; static;
  public
    constructor Create(aCount: Integer; aBallRadius: Single; aLinkDistance: Single;
      const aStartPos: TPointF; aUseRandomColors: Boolean = True);
    procedure Update(const aMousePos: TPointF); override;

    property LinkDistance: Single read FLinkDistance write FLinkDistance;
    property UseFabrik: Boolean read FUseFabrik write FUseFabrik;
    property BallCollision: Boolean read FBallCollision write FBallCollision;
    property AnchorPos: TPointF read FAnchorPos write FAnchorPos;
  end;

implementation

{ TDistanceChainSimulation }

class function TDistanceChainSimulation.RandomColor: TAlphaColor;
begin
  Result := TAlphaColorF.Create(Random(255) / 255, Random(255) / 255, Random(255) / 255, 1).ToAlphaColor;
end;

constructor TDistanceChainSimulation.Create(aCount: Integer; aBallRadius: Single;
  aLinkDistance: Single; const aStartPos: TPointF; aUseRandomColors: Boolean);
var
  i: Integer;
begin
  inherited Create;

  FLinkDistance := aLinkDistance;
  FUseFabrik := False;
  FBallCollision := False;
  FAnchorPos := aStartPos;

  //Main particle not used in chain, but we set it for consistency
  FMainParticle.Pos := aStartPos;
  FMainParticle.Radius := aBallRadius;
  FMainParticle.Color := TAlphaColors.White;

  SetLength(FParticles, aCount);
  for i := 0 to aCount - 1 do
  begin
    FParticles[i].Pos := PointF(aStartPos.X + ((i + 1) * aLinkDistance), aStartPos.Y);
    FParticles[i].Radius := aBallRadius;
    if aUseRandomColors then
      FParticles[i].Color := RandomColor
    else
      FParticles[i].Color := TAlphaColors.White;
  end;
end;

procedure TDistanceChainSimulation.Update(const aMousePos: TPointF);
var
  i, j: Integer;
  toNext, lOffset: TPointF;
  lRadius: Double;
begin
  //First particle follows mouse
  FParticles[0].Pos := aMousePos;

  //Forward pass: each particle constrained to previous
  for i := 1 to High(FParticles) do
    FParticles[i].Pos := TConstraintResolver.Distance(
      FParticles[i].Pos, FParticles[i - 1].Pos, FLinkDistance);

  //FABRIK: backward pass from anchor
  if FUseFabrik then
  begin
    FParticles[High(FParticles)].Pos := FAnchorPos;
    for i := High(FParticles) downto 1 do
      FParticles[i - 1].Pos := TConstraintResolver.Distance(
        FParticles[i - 1].Pos, FParticles[i].Pos, FLinkDistance);
  end;

  //Ball collision
  if FBallCollision then
  begin
    for i := 0 to High(FParticles) do
      for j := i + 1 to High(FParticles) do
      begin
        toNext := FParticles[j].Pos - FParticles[i].Pos;
        lRadius := FParticles[j].Radius + FParticles[i].Radius;
        if toNext.Length <= lRadius then
        begin
          toNext.SetLength(lRadius);
          lOffset := FParticles[j].Pos - FParticles[i].Pos - toNext;
          lOffset := lOffset / 2;
          FParticles[i].Pos := FParticles[i].Pos + lOffset;
          FParticles[j].Pos := FParticles[j].Pos - lOffset;
        end;
      end;
  end;

  //Update main particle to follow first particle (for renderer consistency)
  FMainParticle.Pos := FParticles[0].Pos;
end;

end.
